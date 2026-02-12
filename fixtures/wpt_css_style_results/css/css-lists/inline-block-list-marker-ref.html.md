# css/css-lists/inline-block-list-marker-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/inline-block-list-marker-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:14px/1 monospace; padding:0; margin:0;
}
ol,ul,li { margin:0; padding:0; }
body { margin-left: 40px; }

li { border: 1px solid; display: list-item; }
li::marker { content: counters(list-item, ".") " "; }

.wrap { width: 22ch; }
.m { width: fit-content; }
ib { display:inline-block; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
