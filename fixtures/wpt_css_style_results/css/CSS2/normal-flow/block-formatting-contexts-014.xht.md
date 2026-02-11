# css/CSS2/normal-flow/block-formatting-contexts-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-formatting-contexts-014.xht"
}
```

## style[0]

```css

            #div1
            {
                height: 200px;
                width: 200px;
            }
            div div
            {
                height: 50px;
                width: 50px;
            }
            #div2
            {
                background: blue;
                float: left;
            }
            #div3
            {
                overflow: auto;
            }
            div div div
            {
                background: orange;
                height: 1in;
                width: 1in;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
