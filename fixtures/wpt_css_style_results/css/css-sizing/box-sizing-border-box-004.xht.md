# css/css-sizing/box-sizing-border-box-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-sizing/box-sizing-border-box-004.xht"
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
      box-sizing: border-box;
      min-width: 250px;
      max-width: 350px;
      min-height: 70px;
      max-height: 90px;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
