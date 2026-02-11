# css/css-shadow/has-slotted-functional-flattened-004.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/has-slotted-functional-flattened-004.tentative.html"
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
                    :has-slotted(.slotted-element) {
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
