# css/CSS2/tables/table-backgrounds-bs-table-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/table-backgrounds-bs-table-001-ref.xht"
}
```

## style[0]

```css


	html, body { margin: 0; padding: 0; border: 0; font-size: 16px; }
	body { padding: 15px; }

	/*
	table {
		margin: 0px 3px 2px 4px;
		border-width: 4px 2px 7px 3px;

		border-box height is 145px:
		  border-top: 4px;
		  padding-top: 3px
		  border-spacing: 3px * 6 == 18px
		  cell * 5: == 21px * 5 == 105px
		    border-top: 2px;
		    padding-top: 1px;
		    height: 10px;
		    padding-bottom: 4px;
		    border-bottom: 4px;
		  padding-bottom: 8px;
		  border-bottom: 7px;

		border-box width is 325px:
		  border-left: 3px;
		  padding-left: 6px
		  border-spacing: 2px * 6 == 12px
		  cell * 5: == 59px * 5 == 295px
		    border-left: 3px;
		    padding-left: 3px;
		    width: 50px;
		    padding-right: 2px;
		    border-right: 1px;
		  padding-right: 7px;
		  border-right: 2px;
    */

	div { width: 325px; height: 145px; margin: 0px 3px 2px 4px; }
	div.color { background-color: aqua; }

	div.imagetl, div.imagebr {
		background-image: url(support/repeatable-diagonal-gradient-with-ticks.png);
	}

	div.imagetl { background-position: 3px 4px; }
	div.imagebr { background-position: 323px 138px; }

	
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
