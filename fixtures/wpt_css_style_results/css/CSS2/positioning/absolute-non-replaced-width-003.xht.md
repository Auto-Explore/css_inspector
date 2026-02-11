# css/CSS2/positioning/absolute-non-replaced-width-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-width-003.xht"
}
```

## style[0]

```css

            #containingblock
            {
                border: solid black;
                height: 200px;
                position: relative;
                width: 400px;
            }
            div div
            {
                /*
                  left				:  100px
                +
                  margin-left		:  solve
                +
                  border-left-width :    0px
                +
                  padding-left		:    0px
                +
                  width				:  100px
                +
                  padding-right		:    0px
                +
                  border-right-width:    0px
                +
                  margin-right		:  solve
                +
                  right				: -200px
                =============================
          width of containing block :  400px

                */

                background: red;
                color: blue;
                font: 100px/1em Ahem;
                left: 100px;
                margin-left: auto; /* value is solved to 200px */
                margin-right: auto; /* value is solved to 200px */
                position: absolute;
                right: -200px;
                width: 100px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
