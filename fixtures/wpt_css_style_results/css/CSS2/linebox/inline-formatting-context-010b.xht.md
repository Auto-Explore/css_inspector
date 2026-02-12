# css/CSS2/linebox/inline-formatting-context-010b.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/inline-formatting-context-010b.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  background: white url("support/ruler-v-100px-200px.png") no-repeat;
  margin: 8px 8px 8px 55px;
  /*

    16px : margin collapsing between body's margin-top and p's margin-top == max(8px, 16px)
  +
    20px : first line of p
  +
    20px : second line box of p
  +
    20px : third line box of p
  +
    24px : margin-bottom of p
  =======
   100px

  */
  }

  p
  {
  font: 1em/1.25 serif;
  margin: 1em 0.5em 1.5em;
  }

  div
  {
  background-color: orange; /* The line box will be painted orange */
  font-family: Ahem;
  line-height: 0;
  }

  span
  {
  color: black;
  line-height: 1;
  vertical-align: baseline;
  }

  span#twenty {font-size: 20px; display: none;}

  span#thirty {font-size: 30px; display: none;}

  span#forty {font-size: 40px; display: none;}

  span#fifty {font-size: 50px; display: none;}

  span#sixty {font-size: 60px; display: none;}

  span#eighty {font-size: 80px; display: none;}

  span#one-hundred {font-size: 100px; display: none;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
