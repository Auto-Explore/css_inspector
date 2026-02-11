# css/css-shadow/has-slotted-functional-flattened-003.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/has-slotted-functional-flattened-003.tentative.html"
}
```

## style[0]

```css

                slot {
                    display: block;
                    width: 100px;
                    height: 100px;
                    background-color: green;
                }
                :has-slotted(*) {
                    background-color: red;
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
