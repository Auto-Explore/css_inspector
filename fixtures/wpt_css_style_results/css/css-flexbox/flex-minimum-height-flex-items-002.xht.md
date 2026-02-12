# css/css-flexbox/flex-minimum-height-flex-items-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-minimum-height-flex-items-002.xht"
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
                background-color: green;
                height: 100px;
            }

            #content-100x200 {
                width: 100px;
                height: 200px;
            }
        ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
