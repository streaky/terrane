# Iced UI

`iced-ui` is a Terrane-native declarative application layer over Iced. Application state, event handling, and view composition stay in Terrane; a small maintained Rust renderer translates the resulting `View` tree into Iced widgets.

## Application model

Implement `Application` with three operations:

- `title` returns the native window title.
- `view` builds the current declarative `View` tree.
- mutable `handle` applies an `Event` to application state.

Pass the application to `launch`. The renderer obtains the initial view, forwards typed widget events to `handle`, and rebuilds the view after each event.

## Views

The package currently provides constructors for:

- `text`
- `button` and `indexed-button`
- `checkbox` and `indexed-checkbox`
- `text-input` and `text-input-with-submit`
- `column` and `row`
- `table` for bounded tabular data
- `virtual-table` for continuously scrolling large logical row sets with a bounded caller-supplied row window
- `scroll`
- `container`

`View` has consuming modifiers for appending children, spacing, padding, text size, fill width or height, and horizontal centering. Widget events carry a stable name plus optional text, boolean state, or integer index payloads.

`virtual-table` takes a header, the currently materialized row window, total logical row count, first materialized row, fixed row height, and viewport event name. Native scroll notifications report the first visible logical row through the event's integer index. The application can therefore retain a small overscanned row window while the scrollbar represents the complete data set.

## Projected Iced implementation

The maintained declarative `Application`/`View` API renders its widget graph in Terrane through
projected `iced::widget::text`, `button`, `column`, `row`, `container`, and `scrollable` operations.
The compiler retains Iced's selected higher-ranked `Into<Element<'a, ...>>` terminal contract,
correlates theme and renderer generics, and emits statically typed callback adapters.

## Native boundary

The public application and component model is Terrane code. The maintained Rust boundary starts the
Iced event loop and converts the package's owned `View` and `Event` data transfer objects. Rendering
is delegated back to the Terrane `render-projected` callback; the only inline Rust in that renderer
iterates the host-owned child vector because projected `Vec` fields are not Terrane collection
storage.
The checked-in native inventory is intentionally exhaustive:

- `rust/iced_ui.rs` converts package-owned `View` and `Event` transfer objects and implements the
  object-safe application adapter required to cross the generated Terrane library boundary.
- `rust/host/src/lib.rs` owns Iced's event-loop state, one-shot boot handoff, and higher-ranked
  renderer callback because those are host lifecycle responsibilities rather than projected widget
  construction.
- `src/application.trn` owns widget selection and composition through projected Iced operations.

No widget constructor or widget-to-`Element` conversion remains handwritten in Rust. A future
compiler capability should remove a native item only when its replacement preserves the same host
lifecycle, ownership, and callback contract; this list is the reconciliation baseline for that
cutover.

## Verification boundary

Run the package verification with
`cargo run --quiet --bin terrane -- test packages/iced-ui && cargo test --manifest-path
packages/iced-ui/rust/host/Cargo.toml`. The first command compiles the package-owned
`render-projected` callback and generated higher-ranked Iced adapter; the second runs the headless
host test that boots an application and invokes that renderer boundary once. Automation
intentionally stops before
`iced::Application::run`: that call owns the real desktop event loop and requires a display and user
shutdown. A release smoke run therefore launches the maintained example under a real display,
checks the initial projected view, and closes the window.


Raw Iced widget types, renderer lifetimes, subscriptions, commands, themes, and advanced styling are not exposed through this initial surface. Extend the Terrane `View` and `Event` contracts first when adding generally useful controls; keep direct Rust adapters for host behavior that cannot yet be represented by projected Terrane APIs.
