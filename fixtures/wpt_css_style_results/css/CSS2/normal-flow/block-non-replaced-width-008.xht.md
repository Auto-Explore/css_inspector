# css/CSS2/normal-flow/block-non-replaced-width-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-non-replaced-width-008.xht"
}
```

## style[0]

```css
<![CDATA[
  div {height: 200px;}

  div#containing-block
  {
  border-right: red solid 200px;
  padding-right: 200px;
  width: 0px;
  }

  div#child
  {
  border-right: green solid 200px;
  margin-right: -400px;
  }

  /*
  Calculation of used width for div#child:

    margin-left             :    0px (or auto)
  + border-left-width       :    0px
  + padding-left            :    0px
  + width                   :    auto
  + padding-right           :    0px
  + border-right-width      :  200px
  + margin-right            : -400px
  ====================================
  width of containing block :    0px

  Therefore 'width: auto' must be resolved as 'width: 200px'.
  Therefore div#child's green border-right should overlap perfectly
  div#containing-block's red border-right.
  */
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
