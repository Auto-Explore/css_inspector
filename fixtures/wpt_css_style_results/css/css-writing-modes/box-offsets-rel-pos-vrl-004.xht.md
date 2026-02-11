# css/css-writing-modes/box-offsets-rel-pos-vrl-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/box-offsets-rel-pos-vrl-004.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

  div#statically-positioned-box
    {
      background-color: yellow; /* padding box will be yellow */
      border: orange solid 50px; /* border box will be orange */
      height: 100px; /* a bright green square 100px by 100px image will serve as content box */
      margin-right: 8px;
      padding: 50px;
      position: static;
      width: 100px;
    }

  div.blue-relatively-positioned
    {
      background-color: blue;
      color: white;
      height: 25px;
      position: relative;
      width: 25px;
      writing-mode: horizontal-tb;
    }

  div#top-left
    {
      left: 75px;
      /*
      Calculation of left offset:
          25px (div#top-left's content width)
       +
          50px (div#statically-positioned-box's border-left)
       ==================
          75px
      */

      top: 50px;
      /*
      Calculation of top offset:
        50px (div#statically-positioned-box's border-top)
      ==================
        50px
      */
    }

  div#top-right
    {
      left: 275px;
      /*
      Calculation of left offset:
          25px (div#top-left's content width)
       +
          50px (div#statically-positioned-box's border-left)
       +
         200px (div#statically-positioned-box's padding-box)
       ==================
         275px
      */

      top: 50px;
      /*
      Calculation of top offset:
        50px (div#statically-positioned-box's border-top)
      ==================
        50px
      */
    }

  div#bottom-left
    {
      top: 225px;
      /*
      Calculation of top offset:
          50px (div#statically-positioned-box's border-top)
       +
         200px (div#statically-positioned-box's padding-box height)
       -
          25px (div#bottom-left's content height)
      ==================
         225px
    */

      left: 125px;
      /*
      Calculation of left offset:
          25px (div#top-left's content width)
       +
          25px (div#top-right's content width)
       +
          25px (div#bottom-left's content width)
       +
          50px (div#statically-positioned-box's border-left)
       ==================
         125px
      */
    }

  div#bottom-right
    {
      top: 225px;
      /*
      Calculation of bottom offset:
          50px (div#statically-positioned-box's border-top)
       +
         200px (div#statically-positioned-box's padding-box height)
       -
          25px (div#bottom-right's content height)
      ==================
         225px
    */

      left: 325px;
      /*
      Calculation of left offset:
          25px (div#top-left's content width)
       +
          25px (div#top-right's content width)
       +
          25px (div#bottom-left's content width)
       +
          50px (div#statically-positioned-box's border-left)
       +
         200px (div#statically-positioned-box's padding-box)
       ==================
         325px
      */
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
