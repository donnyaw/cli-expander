# P14-10: Add Tmux-Safe Form Execution Strategy

## Objective

Make interactive form triggers usable from tmux without corrupting the target pane or losing the final command.

## Implementation Status

Completed on branch `feature/tmux-integration` with tag `p14-10`.

## Why This Exists

The current form renderer uses Cursive. Full-screen terminal UIs can be fragile inside tmux, especially when invoked through shell capture or from the wrong pane. The safest tmux design is to run forms inside the popup and inject only the final result into the original pane.

## Scope

- Run picker/form interaction inside tmux popup.
- Keep target pane stable while the form is active.
- Inject completed result back to original pane.
- Document Cursive limitations.
- Do not replace Cursive yet.

## Target Flow

```text
selected pane is active
-> user opens tmux popup picker
-> selected trigger requires a form
-> form runs inside popup
-> final rendered command is produced
-> command is injected into original pane
```

## Implementation Considerations

The current `ce expand` writes the final form result to stdout. For popup form mode, the simplest reliable MVP is to run `ce expand --output tmux --target-pane <pane>` inside the popup. The form UI appears in the popup, and injection happens only after the form returns a final command.

Preferred MVP:

```bash
ce expand "$trigger" --output tmux --target-pane "$target"
```

Avoid adding a separate `ce inject-text` command in this task. It increases API surface area and is not required for the MVP.

If Cursive cannot run correctly in the popup, stop at documentation and manual verification rather than adding a second form renderer in this task. A wizard renderer should be a later dedicated task.

## Cancellation Rule

If the user cancels the form, `ce expand` should exit non-zero and perform no tmux injection. The popup script should preserve that behavior and avoid printing confusing partial output into the target pane.

## Direct Trigger Support

The popup picker should not need to know whether a trigger is text or form-based. It should call the same command for both:

```bash
ce expand "$trigger" --output tmux --target-pane "$target"
```

## Acceptance Criteria

- Form UI does not run in the target pane.
- Completed form output is inserted into the original pane.
- Canceling the form does not inject anything.
- Known limitations are documented.
- Text triggers and form triggers use the same popup command path.

## Test Plan

- Test a simple form trigger from popup.
- Test cancel path.
- Test a choice field.
- Test a cascade/dependent field if available.
- Test target pane remains visually stable while form is active.

## Dependencies

- `P14-06`.

## Follow-Up

Consider a wizard-style form renderer that avoids Cursive alternate-screen behavior for tmux use.
