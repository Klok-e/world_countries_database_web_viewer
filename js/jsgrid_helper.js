let table_name_to_fields = {
    "regions.tera": [
        {name: "region_id", title: "Region ID", type: "number", width: 50},
        {name: "region_name", title: "Region Name", type: "text", width: 100},
        {
            name: "fg_country_name",
            title: "Country Name",
            type: "foreignKeyField",
            width: 100,
            ref_table: "countries.tera",
            ref_column: "name"
        },
        {name: "population", title: "Population", type: "number", width: 50},
        {name: "area_m2", title: "Area M2", type: "number", width: 50},
        {name: "climate", title: "Climate", type: "text", width: 100},
        {
            name: "fg_centre_city_id",
            title: "Central City ID",
            type: "foreignKeyField",
            width: 50,
            ref_table: "cities.tera",
            ref_column: "city_id",
            number: true,
        },
        {type: "my_control"}
    ],
    "cities.tera": [
        {name: "city_id", title: "City ID", type: "number", width: 50},
        {name: "city_name", title: "City Name", type: "text", width: 100},
        {
            name: "fg_region_id", title: "Region ID", type: "foreignKeyField", width: 50,
            ref_table: "regions.tera",
            ref_column: "region_id",
            number: true,
        },
        {type: "my_control"}
    ],
    "continents.tera": [
        {name: "name", title: "Name", type: "text", width: 100},
        {name: "area_m2", title: "Area M2", type: "number", width: 50},
        {type: "my_control"}
    ],
    "countries.tera": [
        {name: "name", title: "Name", type: "text", width: 100},
        {
            name: "fg_continent_name", title: "Continent", type: "foreignKeyField", width: 50,
            ref_table: "continents.tera",
            ref_column: "name"
        },
        {
            name: "fg_capital_city_id", title: "Capital ID", type: "foreignKeyField", width: 50,
            ref_table: "cities.tera",
            ref_column: "city_id",
            number: true,
        },
        {type: "my_control"}
    ],
    "districts.tera": [
        {name: "district_id", title: "District ID", type: "number", width: 50},
        {name: "district_name", title: "District Name", type: "text", width: 100},
        {
            name: "fg_city_id", title: "City ID", type: "foreignKeyField", width: 50,
            ref_table: "cities.tera",
            ref_column: "city_id",
            number: true,
        },
        {type: "my_control"}
    ]
};

let ForeignKeyField = function (config) {
    jsGrid.Field.call(this, config);
};

function close_picker(p) {
    p.css({
        display: "none"
    })
}

function table_picker(value, ref_table, ref_column) {
    if ($("#table_picker").length === 0) {
        $("body").append("<div id='table_picker'>")
    }
    let picker = $("#table_picker");

    let inp = $("<input id='ioerhiaothvebtgel5345rhrly" + ref_table + "" + ref_column + "'>");
    inp.on("click", function () {
        picker.jsGrid({
            autoload: true,
            paging: true,
            pageLoading: true,

            pageSize: 8,
            pageButtonCount: 5,

            width: "40%",
            fields: table_name_to_fields[ref_table].slice(0, -1),

            rowClick: function (item) {
                inp.val(item.item[ref_column]);
                close_picker(picker);
            },

            controller: {
                loadData: function (filter) {
                    return $.ajax({
                        type: "GET",
                        url: "/" + ref_table + "/items?page_index=" + filter.pageIndex + "&page_size=" + filter.pageSize,
                        error: function (jqXHR, textStatus, errorThrown) {
                            console.log(textStatus)
                        }
                    });
                },
            },
        });
        let jthis = $(this);
        picker.css({
            position: "absolute",
            top: jthis.offset().top + jthis.outerHeight() + 0,
            left: jthis.offset().left,
            display: "block"
        });
        let rt = ($(window).width() - (picker.offset().left + picker.outerWidth()));
        if (rt < 0) {
            picker.css({
                left: $(window).width() - picker.outerWidth(),
            });
        }
    });
    inp.val(value);
    return inp;
}

ForeignKeyField.prototype = new jsGrid.Field({
    number: false,
    ref_table: null,
    ref_column: null,

    itemTemplate: function (value) {
        return value;
    },

    insertTemplate: function (value) {
        return this._insert_val = table_picker(value, this.ref_table, this.ref_column);
    },

    editTemplate: function (value) {
        return this._edit_val = table_picker(value, this.ref_table, this.ref_column)
    },

    insertValue: function () {
        if (this.number === false) {
            return this._insert_val.val();
        } else {
            return parseInt(this._insert_val.val());
        }
    },

    editValue: function () {
        if (this.number === false) {
            return this._edit_val.val();
        } else {
            return parseInt(this._edit_val.val());
        }
    }
});
jsGrid.fields.foreignKeyField = ForeignKeyField;

let MyControl = function (config) {
    jsGrid.Field.call(this, config);
};
MyControl.prototype = new jsGrid.fields.control({
    insertTemplate: function (value) {
        let res = jsGrid.fields.control.prototype.insertTemplate.apply(this, value);
        res.click(function () {
            close_picker($("#table_picker"));
            //console.log("insert close");
        });
        //console.log("insert");
        return res;
    },

    editTemplate: function (value) {
        let res = jsGrid.fields.control.prototype.editTemplate.apply(this, value);
        res.click(function () {
            close_picker($("#table_picker"));
            //console.log("edit close");
        });
        //console.log("edit");
        return res;
    },
});
jsGrid.fields.my_control = MyControl;

function create_grid(gridId, page_name) {
    let previousItem;
    $(gridId).jsGrid({
        width: "100%",

        inserting: true,
        editing: true,
        autoload: true,
        paging: true,
        pageLoading: true,

        pageSize: 15,
        pageButtonCount: 5,

        onItemUpdating: function (args) {
            previousItem = args.previousItem;
        },

        controller: {
            loadData: function (filter) {
                return $.ajax({
                    type: "GET",
                    url: "/" + page_name + "/items?page_index=" + filter.pageIndex + "&page_size=" + filter.pageSize,
                    error: function (jqXHR, textStatus, errorThrown) {
                        console.log(textStatus)
                    }
                });
            },

            insertItem: function (item) {
                let d = $.Deferred();
                console.log(item);
                $.ajax({
                    type: "POST",
                    url: "/" + page_name + "/items",
                    data: JSON.stringify(item),
                    contentType: "application/json",
                    error: function (jqXHR, textStatus, errorThrown) {
                        console.log(textStatus);
                        $(gridId).jsGrid("clearInsert");
                        d.reject()
                    },
                    success: function (data, status, jqXHR) {
                        d.resolve(data)
                    },
                });
                return d.promise()
            },

            updateItem: function (item) {
                let d = $.Deferred();
                console.log(item);
                $.ajax({
                    type: "PUT",
                    url: "/" + page_name + "/items",
                    data: JSON.stringify({old: previousItem, new: item}),
                    contentType: "application/json",
                    error: function (jqXHR, textStatus, errorThrown) {
                        console.log(textStatus);
                        $(gridId).jsGrid("clearInsert");
                        d.reject()
                    },
                    success: function (data, status, jqXHR) {
                        if (jqXHR.status === 278) {
                            $(gridId).jsGrid("clearInsert");
                            show_warning("Database Error", data.error_msg);
                            d.reject();
                        } else {
                            d.resolve(data)
                        }
                    },
                });
                return d.promise()
            },

            deleteItem: function (item) {
                let d = $.Deferred();
                console.log(item);
                $.ajax({
                    type: "DELETE",
                    url: "/" + page_name + "/items",
                    data: JSON.stringify(item),
                    contentType: "application/json",
                    error: function (jqXHR, textStatus, errorThrown) {
                        console.log(textStatus);
                        $(gridId).jsGrid("clearInsert");
                        d.reject()
                    },
                    success: function (data, status, jqXHR) {
                        d.resolve(data)
                    },
                });
                return d.promise()
            },
        },

        fields: table_name_to_fields[page_name]
    });
}

function show_warning(header, message) {
    let prot = $("#warning_card_prot");
    let new_card = prot.clone();
    new_card.removeAttr("style");
    new_card.insertAfter(prot);
    new_card.find(".card-title").text(header);
    new_card.find(".card-body").text(message);
}