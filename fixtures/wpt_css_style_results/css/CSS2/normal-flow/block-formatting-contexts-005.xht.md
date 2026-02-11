# css/CSS2/normal-flow/block-formatting-contexts-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-formatting-contexts-005.xht"
}
```

## style[0]

```css

            div
            {
                height: 1in;
            }
            #div1
            {
                border-left: solid 5px blue;
            }
            div div
            {
                border-left: solid 5px orange;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
