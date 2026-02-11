# css/CSS2/visufx/clip-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visufx/clip-001.xht"
}
```

## style[0]

```css
<![CDATA[
  #red-overlapped-layer
  {
  background-color: red;
  height: 100px;
  position: absolute;
  width: 100px;
  }

  #red-parent
  {
  background-color: red;
  clip: rect(0, 1px, 1px, 0);
  height: 2px;
  position: absolute;
  width: 2px;
  }

  #green-child
  {
  background-color: green;
  height: 100px;
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
