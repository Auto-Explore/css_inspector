# css/css-will-change/will-change-stacking-context-z-index-1.html

```json
{
  "format_version": 3,
  "file": "css/css-will-change/will-change-stacking-context-z-index-1.html"
}
```

## style[0]

```css

html, body { margin: 0; padding: 0; }
div { width: 100px; height: 100px }
#wc { will-change: z-index; background: red; position: relative }
#child { position: absolute; top: 0; left: 0; z-index: -1; background: green }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
