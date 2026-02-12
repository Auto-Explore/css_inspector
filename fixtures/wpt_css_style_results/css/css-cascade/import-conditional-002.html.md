# css/css-cascade/import-conditional-002.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/import-conditional-002.html"
}
```

## style[0]

```css

  @import "support/test-red.css";
  @import "support/test-green.css"
    supports(display: block);
  @import "support/test-red.css"
    supports(foo: bar);
  div {
    box-sizing: border-box;
    width: 100px;
    height: 100px;
    padding: 5px; /* Avoids text antialiasing issues */
    background: red;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
