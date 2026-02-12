# css/CSS2/normal-flow/block-non-replaced-width-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-non-replaced-width-003.xht"
}
```

## style[0]

```css

            div
            {
                direction: ltr;
            }
            #div1
            {
                background: orange;
                width: 100px;
            }
            div div
            {
                background-color: orange;
                border-left: 10px solid orange;
                border-right: 10px solid orange;
                height: 30px;
                margin-left: 10px;
                margin-right: 10px;
                padding-left: 10px;
                padding-right: 10px;
                width: 100px;
            }

			/*
			In this test, the used margin-right should be -50px so
			that the equation remains balanced.
			*/

            #div2
            {
                background-color: blue;
                height: 30px;
                position: absolute;
                top: 82px;
                width: 150px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
