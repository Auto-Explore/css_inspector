# css/css-shadow/has-slotted-functional-flattened-006.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/has-slotted-functional-flattened-006.tentative.html"
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
                    :has-slotted(div) + :has-slotted(div) {
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
