# css/CSS2/box-display/containing-block-019.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/box-display/containing-block-019.xht"
}
```

## style[0]

```css

            div
            {
                border: solid black;
                padding: 1in;
                position: absolute;
                width: 0;
            }
            span
            {
                display: block;
            }
            #span1
            {
                direction: ltr;
            }

			/*
			The span#span1 element determines, conditions (§10.3.7 and
			§10.6.4) the 'top: auto' and 'left: auto' coordinates while the
			wrapping div's padding box forms the containing block
			geometry/area.
			*/

            span span
            {
                background: blue;
                height: 1in;
                left: auto;
                position: absolute;
                top: auto;
                width: 1in;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
