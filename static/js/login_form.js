$(document).ready(function () {
    let login_form = $(".login_form");
    let login_button = $(".login");
    login_form.hide();
    login_button.click(function () {
        login_form.show()
    });
});