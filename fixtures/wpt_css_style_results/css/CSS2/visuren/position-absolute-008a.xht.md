# css/CSS2/visuren/position-absolute-008a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visuren/position-absolute-008a.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  height: 100px;
  width: 100px;
  }

  div#rel-pos-container
  {
  position: relative;
  width: auto;
  }

  div#abspos-green-overlapping
  {
  background-color: green;
  float: right;
  left: auto;
  position: absolute;
  top: auto;
  }

  div#static-red-overlapped
  {
  background-color: red;
  position: static;
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
