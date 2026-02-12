# css/css-sizing/box-sizing-border-box-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-sizing/box-sizing-border-box-003.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      width: 300px;
      height: 400px;
      border: 2px solid black;
      position: absolute;
      left: 25px;
      top: 25px;
      background-color: red;
    }

    .box-sized {
      box-sizing: border-box;
      width: 50%;
      height: 100%;
      z-index: 1;
      float: left;
      border: 5px solid black;
      padding: 25px 10px;
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
