# css/CSS2/text/white-space-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/text/white-space-008.xht"
}
```

## style[0]

```css
<![CDATA[
  #test-overlapping-green
  {
  background-color: lime;
  white-space: pre;
  }

  #reference-overlapped-red
  {
  background-color: red;
  left: 0;
  position: absolute;
  top: 0;
  width: 100%;
  z-index: -1;
  }

  #relatively-positioned-wrapper
  {
  position: relative;
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
