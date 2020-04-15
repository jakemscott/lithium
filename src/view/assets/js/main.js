let ID = [];
$(document).ready( function () {
    // load all names for later comparison
    $('.table td.name').each(function() {
        ID.push($(this).html())
    });

    // if we are in the normal view load the appropriate table
    let viewTable = $('table#view').DataTable({
        dom: 't',
        deferRender: true,
        scrollY: 'calc(100vh - 305px)',
        scroller: true,
        responsive: true
    });

    // if we are looking at the admin view load the admin table
    let adminTable = $('table#edit').DataTable({
        dom: 't',
        deferRender: true,
        scrollY: 'calc(100vh - 305px)',
        scroller: true,
        columnDefs: [
            {
                targets: [0, 3, 4, 5],
                orderable: false
            },
            {
                targets: [3, 4, 5],
                width: "15px"
            }
        ]
    });

    // add and modify the search behaviour of the table
    $.fn.dataTable.ext.search.push(
        function( settings, data, dataIndex ) {
            // only apply filter to the displayed table
            let input = $('#search').val().toLowerCase();
            if (input.length <= 0) return true;
            return !!(~data[0].toLowerCase().indexOf(input) || ~data[1].toLowerCase().indexOf(input));
        }
    );
    $('#search').keyup(function() {
        adminTable.draw();
        viewTable.draw();
    });

    //handle any warnings that appear
    let alert = $('.alert');
    if (alert[0]) {
        $(alert).fadeIn();
        setTimeout(function() {
            alert.alert("close");
        })
    }
});

// close alerts by clicking anywhere on body
$("button[data-dismiss='alert']").click(function(){
    $(".alert").alert("close");
});

// edit existing entry
$('button[data-target="#modal_edit"]').on('click', function() {
    let row = $(this).closest('tr');
    let id = row.find('td:eq(0)').text();
    let name = row.find('td:eq(1)').text();
    let link = row.find('td:eq(2)').text().trim();

    let form = $('.modal#modal_edit');
    form.find('#edit_id').val(id);
    form.find('#edit_name').val(name);
    form.find('#edit_link').val(link);
});

$('button[data-target="#modal_new"]').on('click', function() {
    let form = $('.modal#modal_new');
    let name = Utilities.random_characters(7);
    while (ID.includes(name)) name = Utilities.random_characters(7);

    form.find('#new_name').val(name);
});

$('button[data-target="#modal_delete"]').on('click', function() {
    let row = $(this).closest('tr');
    let id = row.find('td:eq(0)').text();

    let form = $('.modal#modal_delete');
    form.find('#delete_id').val(id);
});

$('button[data-target="#modal_visible"]').on('click', function() {
    let row = $(this).closest('tr');
    let id = row.find('td:eq(0)').text();

    let form = $('.modal#modal_visible');
    form.find('#vis_id').val(id);
});