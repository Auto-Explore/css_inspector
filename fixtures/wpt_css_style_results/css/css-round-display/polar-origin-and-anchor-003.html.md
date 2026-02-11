# css/css-round-display/polar-origin-and-anchor-003.html

```json
{
  "format_version": 3,
  "file": "css/css-round-display/polar-origin-and-anchor-003.html"
}
```

## style[0]

```css

		    .container {
		      width: 200px;
		      height: 200px;
		      border: medium solid black;
		    }
		    .item {
		      position: absolute;
		      polar-origin: bottom;
		      polar-anchor: left;
		      polar-distance: 0px;
			  polar-angle: 90deg;
		      width: 50px;
		      height: 50px;
		      background-color: red;
		    }
		  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “polar-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “polar-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “polar-distance”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “polar-angle”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
