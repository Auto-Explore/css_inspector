# css/css-multicol/multicol-basic-004.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-basic-004.html"
}
```

## style[0]

```css

		.multicol-wrapper>*{
			font: 20px/1 Ahem;
		}

		div.multicol-wrapper{
			border: thin solid black;
			display: inline-block;
			margin: 1em auto;
			width: 360px;
		}

		.multicol-basic-ref{
			background: yellow;
			width: 360px;
			column-width: 120px;
			column-gap: 0;
			column-rule: none;
		}

		.multicol-basic-ref-item{
			background: #000;
		}

		.item-1{
			background: purple;
			color: purple;
		}

		.item-2{
			background: orange;
			color: orange;
		}

		.item-3{
			background: blue;
			color: blue;
		}
	
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
