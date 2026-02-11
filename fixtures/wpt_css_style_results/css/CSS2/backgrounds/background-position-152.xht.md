# css/CSS2/backgrounds/background-position-152.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-152.xht"
}
```

## style[0]

```css
<![CDATA[
  #nearest-positioned-ancestor
  {
  background-color: green; /* padding box will be green */
  background-image: url("support/100x100-red.png");
  background-position: 14% 84%;
  background-attachment: scroll;
  background-repeat: no-repeat;
  height: 100px; /* therefore padding box is 300px tall */
  padding: 100px;
  position: relative;
  width: 200px; /* therefore padding box is 400px wide */
  }

  /*

  In this testcase, the point coordinates of the background-image
  are (14px, 84px) and such inner point of the background-image will
  be placed at the point 14% across and 84% down the padding box of
  div#nearest-positioned-ancestor.

  Calculations of 100x100-red position coordinates within #nearest-positioned-ancestor
  ====================================================================================

  along the horizontal axis
  -------------------------

    56px (14% of #nearest-positioned-ancestor's padding box width)
  - 14px (14% of 100x100-red's width)
  ------------------
    42px (overlapping green box's left offset within #nearest-positioned-ancestor)

   42px represents exactly 10.5% of #nearest-positioned-ancestor's padding box
   width (400px)

  along the vertical axis
  -----------------------

   252px (84% of #nearest-positioned-ancestor's padding box height)
  - 84px (84% of 100x100-red's height)
  ------------------
   168px (overlapping green box's top offset within #nearest-positioned-ancestor)

   168px represents exactly 56% of #nearest-positioned-ancestor's padding box
   height (300px)
  */

  #overlapping-abs-pos-green-box
  {
  left: 10.5%; /* == 42px */
  position: absolute;
  top: 56%; /* == 168px */
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
