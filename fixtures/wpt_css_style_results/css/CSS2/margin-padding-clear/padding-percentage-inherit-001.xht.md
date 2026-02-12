# css/CSS2/margin-padding-clear/padding-percentage-inherit-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-percentage-inherit-001.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  p
  {
  font: 1em/1.25 serif;
  margin: 1em 0em;
  }

  div#grand-parent {width: 300px;}

  div#parent
  {
  background-color: green;
  height: 150px;
  padding: 20%; /* 20% of 300px == 60px; */
  width: 150px;
  }

  div#child
  {
  padding: inherit;
  /*
  computes to 20% which is then applied
  on #child's containing block width: 20% of 150px == 30px
  */
  }

  div#abs-pos-overlapping-green
  {
  left: 98px;
  /*
       8px (body's margin-left)
    +
      60px (#parent's padding-left)
   +
      30px (#child's padding-left)
  ==================
      98px
  */
  position: absolute;
  top: 162px;
  }

  /*
      max(8px, 16px) (margin collapsing between body's margin-top and p's margin-top)
   +
      20px (first line)
   +
      20px (second line)
   +
      16px (p's margin-bottom)
   +
      60px (#parent's padding-top)
   +
      30px (#child's padding-top)
  ==================
     162px
  */
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
