# css/css-shadow/has-slotted-functional-007.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/has-slotted-functional-007.tentative.html"
}
```

## style[0]

```css

        [name] {
          display: block;
          width: 100px;
          height: 100px;
          background-color: red;
        }
        :has-slotted(div + div) {
          background-color: green;
        }
      
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
