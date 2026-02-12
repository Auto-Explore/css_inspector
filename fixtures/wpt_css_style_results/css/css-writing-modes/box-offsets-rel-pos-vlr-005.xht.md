# css/css-writing-modes/box-offsets-rel-pos-vlr-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/box-offsets-rel-pos-vlr-005.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
    }

  div#statically-positioned-box
    {
      background-color: yellow; /* padding box will be yellow */
      border: orange solid 50px; /* border box will be orange */
      height: 100px; /* a bright green square 100px by 100px image will serve as content box */
      margin-left: 8px;
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
      right: 250px;
      /*
      Calculation of right offset:
          50px (div#statically-positioned-box's border-right)
       +
         200px (div#statically-positioned-box's padding-box width)
       ==================
         250px
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
      right: 100px;
      /*
      Calculation of right offset:
          25px (div#top-left's content width)
       +
          25px (div#top-right's content width)
       +
          50px (div#statically-positioned-box's border-right)
       ==================
         100px
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

      right: 300px;
      /*
      Calculation of right offset:
          25px (div#top-left's content width)
       +
          25px (div#top-right's content width)
       +
          50px (div#statically-positioned-box's border-right)
       +
         200px (div#statically-positioned-box's padding-box width)
       ==================
         300px
      */
    }

  div#bottom-right
    {
      top: 225px;
      /*
      Calculation of top offset:
          50px (div#statically-positioned-box's border-top)
       +
         200px (div#statically-positioned-box's padding-box height)
       -
          25px (div#bottom-right's content height)
      ==================
         225px
    */

      right: 150px;
      /*
      Calculation of right offset:
          25px (div#top-left's content width)
       +
          25px (div#top-right's content width)
       +
          25px (div#bottom-left's content width)
       +
          25px (div#bottom-right's content width)
       +
          50px (div#statically-positioned-box's border-left)
       ==================
         150px
      */
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
