# css/CSS2/tables/table-backgrounds-bc-column-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/table-backgrounds-bc-column-001-ref.xht"
}
```

## style[0]

```css


	html, body { margin: 0; padding: 0; border: 0; font-size: 16px; }
	body { padding: 15px; }

	/*
	table {
		margin: 0px 3px 2px 4px;
		border-width: 4px 2px 8px 6px;

		height is 97px:
		  (2px of border outside height)
		  extra border width for top row: 0
		  cell * 5: == 19px * 5 == 95px
		    border-top: 2px;
		    padding-top: 1px;
		    height: 10px;
		    padding-bottom: 4px;
		    border-bottom: 2px;
		  extra border width for top row: 2px
		  (4px of border outside height)

		width is 287px:
		  (3px of border outside width)
		  extra border width for left column: 2px
		  cell * 5: == 57px * 5 == 285px
		    border-left: 1px;
		    padding-left: 3px;
		    width: 50px;
		    padding-right: 2px;
		    border-right: 1px;
		  (1px of border outside width)
    */

	div { width: 57px; height: 97px; margin: 2px 121px 8px 123px; }
	div.color { background-color: aqua; }

	div.imagetl, div.imagebr {
		background-image: url(support/repeatable-diagonal-gradient-with-ticks.png);
	}

	div.imagetl { background-position: 0 0; }
	div.imagebr { background-position: 57px 97px; }

	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
