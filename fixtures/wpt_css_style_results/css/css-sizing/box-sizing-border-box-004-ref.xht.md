# css/css-sizing/box-sizing-border-box-004-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-sizing/box-sizing-border-box-004-ref.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      min-width: 500px;
      max-width: 700px;
      min-height: 70px;
      max-height: 90px;
      border: 2px solid black;
      position: absolute;
      left: 25px;
      top: 25px;
      background-color: red;
    }

    .box-sized {
      min-width: 240px;
      max-width: 340px;
      min-height: 60px;
      max-height: 80px;
      z-index: 1;
      float: left;
      border: 5px solid black;
    }

    #one {
      background-color: green;
    }

    #two {
      background-color: blue;
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
