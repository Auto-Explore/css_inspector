# css/compositing/Blending_in_a_group_with_filter.html

```json
{
  "format_version": 3,
  "file": "css/compositing/Blending_in_a_group_with_filter.html"
}
```

## style[0]

```css

        #redSquare {
			mix-blend-mode: lighten;
        }
        #svgMain {
            top: 100px;
            left: 100px;
            width: 200px;
            height: 200px;
			background: blue;
        }
		#group{
			filter: alpha(opacity=50,finishopacity=50,style=2);
		}
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
