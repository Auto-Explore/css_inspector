# css/css-shadow/slotted-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/slotted-nested.html"
}
```

## style[0]

```css

      /* This is not expected to match */
      .container ::slotted(p) {
        color: red !important;
      }

      /* This _is_ expected to match */
      #nested ::slotted(p) {
        background-color: green;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

      .container ::slotted(p) {
        color: green;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
