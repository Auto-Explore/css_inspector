# css/css-flexbox/flex-minimum-height-flex-items-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-minimum-height-flex-items-011.xht"
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
                display: flex;
                flex-basis: 0;
                color: green;
                background-color: green;
                font: 50px/1 Ahem;
            }

            #content-100x100 {
                width: 100px;
                height: 100px;
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
