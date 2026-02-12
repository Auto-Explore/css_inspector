# css/css-flexbox/flex-outer-flexbox-column-recalculate-height-on-resize-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-outer-flexbox-column-recalculate-height-on-resize-001.html"
}
```

## style[0]

```css

body,
html {
  height: 100%;
}

.OuterFlexbox {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.InnerFlexbox {
  display: flex;
  flex-direction: column;
  max-height: 100%;
  outline: 1px blue solid;
}

.InnerFlexbox-body {
  flex: 1 1 auto;
  overflow-y: hidden;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
