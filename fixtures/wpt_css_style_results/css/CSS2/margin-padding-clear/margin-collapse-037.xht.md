# css/CSS2/margin-padding-clear/margin-collapse-037.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-037.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  p {margin: 8px 0px 0px;}

  div#reference-overlapping-green
  {
  background-color: green;
  height: 100px;
  left: 8px;
  position: absolute;
  right: 8px;
  }

  #test-overlapped-red
  {
  background-color: red;
  min-height: 50px;
  }

  #last-child
  {
  height: 100px;
  margin-bottom: 100px;
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
