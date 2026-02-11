# css/css-flexbox/flexbox-flex-direction-default.htm

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-flex-direction-default.htm"
}
```

## style[0]

```css


		/* Test specific Styles */

			.flex-wrapper {
				display: flex;
				flex-wrap: wrap;
			}

		/* Test-series styles */

			.flex-wrapper {
				width:  120px;
				height: 120px;

				/* should only display on error */
				background: red;

				/* Enforce writing mode */
				direction: ltr;
				writing-mode: horizontal-tb;
			}

			.flex-wrapper div {
				width:  38px;
				height: 38px;

				background: green;
				border: 1px solid limegreen;

				color: white;
				line-height: 40px;
				text-align: center;
				vertical-align: middle;
			}
		
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
