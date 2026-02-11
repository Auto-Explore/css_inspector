# css/CSS2/normal-flow/inline-block-non-replaced-height-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/inline-block-non-replaced-height-002.xht"
}
```

## style[0]

```css

            #div1
            {
                line-height: 0;
                position: relative;
            }
            #div2
            {
                background: blue;
            }

			/*
			The test relies and assumes that div#div2's
			computed height will be and should be
			equal to the its used line box height since it has
			only 1 inline-level element which is contributing to
			determine its line box height.
			*/

            #div2 div
            {
                display: inline-block;
                height: 0;
                margin: 0.5in 0;
            }
            div div
            {
                width: 1in;
            }
            #div3
            {
                background: orange;
                left: 1in;
                height: 1in;
                position: absolute;
                top: 0;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
