# css/CSS2/zindex/z-index-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zindex/z-index-015.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  height: 100px;
  position: absolute;
  width: 100px;
  }

  #invalid-zindex
  {
  background-color: red;
  z-index: 2.5;
  }

  #valid-zindex
  {
  background-color: green;
  z-index: auto;
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
