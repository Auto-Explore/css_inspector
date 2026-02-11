# css/css-flexbox/flex-minimum-height-flex-items-008.xht

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-minimum-height-flex-items-008.xht"
}
```

## style[0]

```css
<![CDATA[
            #reference-overlapped-red {
                position: absolute;
                background-color: red;
                width: 100px;
                height: 100px;
                z-index: -1;
            }

            #constrained-flex {
                display: flex;
                flex-direction: column;
                width: 100px;
                height: 10px;
            }

            #test-flex-item-overlapping-green {
                width: 100px;
            }
        ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
