var ForeignKeyField = function (config) {
    jsGrid.Field.call(this, config);
};

ForeignKeyField.prototype = new jsGrid.Field({
    itemTemplate: function (value) {
        //return new Date(value).toDateString();
        return value;
    },

    insertTemplate: function (value) {
        $("<input>").jsGrid({
            width: 200,
            height: 200,

            data: [1, 2, 3, 4]
        });
        console.log("insertTemplate");
        //return this._insertPicker = $("<input>").datepicker({ defaultDate: new Date() });
        return this._insert_val = value
    },

    editTemplate: function (value) {
        let picker_grid = $("<div>");
        document.body.appendChild(picker_grid.get(0));

        picker_grid.jsGrid({
            width: 200,
            height: 200,

            data: [1, 2, 3, 4]
        });
        console.log("editTemplate");
        let inp = $("<input>");
        inp.value = value;
        return inp
    },

    insertValue: function () {
        console.log("insertValue");
        //return this._insertPicker.datepicker("getDate").toISOString();
        return this._insert_val;
    },

    editValue: function () {
        console.log("editValue");
        //return this._editPicker.datepicker("getDate").toISOString();
        return this._edit_val;
    }
});
jsGrid.fields.foreignKeyField = ForeignKeyField;

function create_grid(gridId, page_name, fields) {
    let previousItem;
    $(gridId).jsGrid({
        width: "100%",

        inserting: true,
        editing: true,
        sorting: true,
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

        fields: fields
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