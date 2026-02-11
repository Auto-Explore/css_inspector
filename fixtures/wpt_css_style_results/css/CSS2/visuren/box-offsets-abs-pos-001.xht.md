# css/CSS2/visuren/box-offsets-abs-pos-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visuren/box-offsets-abs-pos-001.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  #nearest-positioned-ancestor
  {
  background-color: yellow; /* padding box will be yellow */
  border: orange solid 50px; /* border box will be orange */
  height: 100px; /* a bright green square 100px by 100px image will serve as content box */
  margin-left: 100px;
  padding: 50px;
  position: relative;
  top: auto;
  width: 100px;
  }

  div > div
  {
  background-color: blue;
  color: white;
  height: 25px;
  position: absolute;
  width: 25px;
  }

  div#top-left
  {
  left: 0px;
  top: 0px;
  }

  div#top-right
  {
  right: 0px;
  top: 0px;
  }

  div#bottom-left
  {
  bottom: 0px;
  left: 0px;
  }

  div#bottom-right
  {
  bottom: 0px;
  right: 0px;
  }
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
