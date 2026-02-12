# css/selectors/invalidation/host-has-shadow-tree-element-at-subject-position.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/host-has-shadow-tree-element-at-subject-position.html"
}
```

## style[0]

```css

        :host:has(.descendant) { color: green; }
        :host:has(> .child) { color: blue; }
        :host:has(~ .sibling) { color: yellow; }
        :host:has(:is(.ancestor .descendant)) { color: purple; }
        :host:has(.descendant):has(> .child) { color: pink; }
        :host-context(.host_context):has(> .child > .grand_child) { color: ivory; }
        :host(.host_context):has(> .child > .grand_child) { color: skyblue; }
        :host:has(> .child > .grand_child):host(.host_context):has(> .child > .descendant) { color: lightgreen; }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
