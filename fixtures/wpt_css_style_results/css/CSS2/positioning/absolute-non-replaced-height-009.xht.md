# css/CSS2/positioning/absolute-non-replaced-height-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-height-009.xht"
}
```

## style[0]

```css

            #div1
            {
                position: relative;
            }
            div div
            {
                background: blue;
                bottom: auto;
                color: orange;
                font: 100px/1 Ahem;
                height: auto;
                margin-bottom: auto;
                margin-top: auto;
                position: absolute;
                top: 25px;
                width: 200px;
            }

  <!--
  "
  height' and 'bottom' are 'auto' and 'top' is not 'auto',
  then the height is based on the content per 10.6.7,
  set 'auto' values for 'margin-top' and 'margin-bottom' to 0,
  and solve for 'bottom'
  "
  http://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-height

  In this test,
  top used value will be 25px
  height used value will be 100px
  vertical margins will be 0px
  bottom used value will be -125px
  because the computed height of containing block is 0px.
  -->

        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
