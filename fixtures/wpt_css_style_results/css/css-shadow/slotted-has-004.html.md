# css/css-shadow/slotted-has-004.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/slotted-has-004.html"
}
```

## style[0]

```css

        slot {
          display: block;
          width: 100px;
          height: 100px;
          background-color: red;
        }
        ::slotted(:not(:has(:first-child:last-child))) {
          display: block;
          width: 100px;
          height: 100px;
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
