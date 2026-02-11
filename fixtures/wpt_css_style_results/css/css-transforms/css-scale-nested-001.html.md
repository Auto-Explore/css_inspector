# css/css-transforms/css-scale-nested-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/css-scale-nested-001.html"
}
```

## style[0]

```css

	div{
		position: absolute;
	}
		.greenSquare {
  			width: 100px;
            height: 100px;
            background: green;
        }
        .parentredSquare {
            width: 100px;
            height: 100px;
            background: red;
			transform: scale(0);
        }
		.childredSquare {
            top: 25px;
            left: 25px;
            width: 50px;
            height: 50px;
	        background: red;
        }
		p{
		padding-top: 130px;
		}
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
