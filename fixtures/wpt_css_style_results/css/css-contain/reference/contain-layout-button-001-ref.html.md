# css/css-contain/reference/contain-layout-button-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/reference/contain-layout-button-001-ref.html"
}
```

## style[0]

```css

div.fakeButton {
  display: inline-block;
  border: 5px solid green;
  padding: 0;
  margin-bottom: -5px;
  color: transparent;
  width: 0;
  height: 0px;
  /* Layout containment creates a stacking context, the following lines simuluate the same in the reference file. */
  position: relative;
  z-index: 1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
