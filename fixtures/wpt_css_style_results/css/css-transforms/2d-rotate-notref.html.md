# css/css-transforms/2d-rotate-notref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/2d-rotate-notref.html"
}
```

## style[0]

```css

			article, svg{
				position: absolute;
				top: 220px;
				left: 60px;
			}
			article{
				border: 10px solid green;
				display: block;
				height: 100px;
				width: 100px;
				z-index: 2;
				cursor:pointer;
			}
			section article{
				transform: rotate(30deg);
				transform-origin: 19% 197%;
			}

		
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

				rect{
					stroke-width: 10;
					stroke: red;
					fill: none;
		   	 	}
			    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “stroke-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
