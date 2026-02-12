# css/selectors/invalidation/host-has-shadow-tree-element-at-nonsubject-position.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/host-has-shadow-tree-element-at-nonsubject-position.html"
}
```

## style[0]

```css

        div { color: red; }
        :host:has(.descendant) .subject { color: green; }
        :host:has(> .child) .subject { color: blue; }
        :host:has(~ .sibling) .subject { color: yellow; }
        :host:has(:is(.ancestor .descendant)) .subject { color: purple; }
        :host:has(.descendant):has(> .child) .subject { color: pink; }
        :host-context(.host_context):has(> .child > .grand_child) .subject { color: ivory; }
        :host(.host_context):has(> .child > .grand_child) .subject { color: skyblue; }
        :host:has(> .child > .grand_child):host(.host_context):has(> .child > .descendant) .subject { color: lightgreen; }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
