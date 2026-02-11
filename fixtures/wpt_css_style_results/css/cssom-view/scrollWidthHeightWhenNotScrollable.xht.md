# css/cssom-view/scrollWidthHeightWhenNotScrollable.xht

```json
{
  "format_version": 3,
  "file": "css/cssom-view/scrollWidthHeightWhenNotScrollable.xht"
}
```

## style[0]

```css
<![CDATA[
            #elemSimple, #elemOverflow, #elemNestedOverflow {
                border:1px solid black;
                width:200px;
                height:40px;
                padding-bottom:50px;
                padding-right:40px;
            }
            #elemSimple > div {
                background:yellow;
                width:60px;
                height:30px;
            }
            #elemOverflow > div {
                background:yellow;
                width:250px;
                height:150px;
            }
            #elemNestedOverflow > div {
                background:yellow;
                width:60px;
                height:30px;
            }
            #elemNestedOverflow > div > div {
                background:blue;
                width:250px;
                height:150px;
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
