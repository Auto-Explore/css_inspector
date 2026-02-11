# css/css-transforms/perspective-untransformable-no-stacking-context.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-untransformable-no-stacking-context.html"
}
```

## style[0]

```css

* { margin: 0; padding: 0; }
div, span { width: 100px; height: 100px }
#perspective { background: green; padding-top: 100px; perspective: 100px; }
#child { display:inline-block; z-index: -1; position:absolute; background: red; }
#spacer { display:inline-block; }
#wrapper { overflow:hidden }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
