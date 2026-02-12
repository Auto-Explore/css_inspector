# css/CSS2/margin-padding-clear/margin-percentage-inherit-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-percentage-inherit-001.xht"
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

  div#grand-parent {width: 400px;}

  div#parent
  {
  background-color: green;
  border: white solid 3px;
  /*
  Such border prevents margin collapsing between
  #child's vertical margin and #parent's vertical margins
  */
  margin: 15%; /* 15% of 400px == 60px; */
  width: 200px;
  }

  div#child
  {
  margin: inherit;
  /*
  computes to 15% which is then applied on #child's
  containing block width: 15% of 200px == 30px
  */
  }

  div#abs-pos-overlapping-green
  {
  left: 101px;
  position: absolute;
  top: 149px;
  }

  /*
       8px (body's margin-left)
   +  60px (#parent's margin-left)
   +   3px (#parent's border-left)
   +  30px (#child's margin-left)
  ==================
     101px
  */

  /*
      16px (max(8px, 16px): body's margin-top collapses with p's margin-top)
  +   20px (first line)
  +   20px (second line)
  +   60px (max(16px, 60px): p's margin-bottom collapses with #parent's margin-top)
  +    3px (#parent's border-top)
  +   30px (#child's margin-top)
  ==================
     149px
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
