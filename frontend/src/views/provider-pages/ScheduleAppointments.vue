<template>
  <div class="scheduleAppointments">
    <v-card elevation="0" class="mx-auto mt-5" max-width="60%">
        <h1 class="text-center mt-4">Schedule Appointments</h1>
        <div>
          <div>
            <v-data-table
                :headers="headers"
                :items="patients"
                item-key="name"
                class="elevation-1"
                :search="search"

            >

              <template v-slot:top>

                <v-text-field
                    v-model="search"
                    label="Search"
                    class="mx-4"

                ></v-text-field>

              </template>

              <template v-slot:body.append>

                <tr>


                  <td></td>

                  <td colspan="4"></td>


                </tr>

              </template>

            </v-data-table>

          </div>
        </div>
   </v-card>
  </div>
</template>

<script>
export default {
  data () {
    return {
      search: '',
      patientID: '',
      patients: [
        {
          name: 'Braxton Meyer',
          patientID: 159,
          date: "10/30/2021",
          time: "8:00 AM",

        },

        {
          name: 'Kieran Clark',
          patientID: 237,
          date: "10/30/2021",
          time: "8:30 AM",

        },
        {
          name: 'Alex Curran',
          patientID: 262,
          date: "10/30/2021",
          time: "9:00 AM",

        },
        {
          name: 'Dillon Kaim',
          patientID: 305,
          date: "10/30/2021",
          time: "9:30 AM",

        },

      ],
    }
  },
  computed: {
    headers () {
      return [
        {
          text: 'Patient',
          align: 'start',
          sortable: false,
          value: 'name',
        },
        {
          text: 'PatientID',
          value: 'patientID',
          filter: value => {
            if (!this.patientID) return true

            return value < parseInt(this.patientID)
          },
        },
        { text: 'Date', value: 'date' },
        { text: 'Time', value: 'time' },


      ]
    },
  },
  methods: {
    filterOnlyCapsText (value, search, item) {
      return value != null &&
          search != null &&
          typeof value === 'string' &&
          value.toString().toLocaleUpperCase().indexOf(search) !== -1
    },
  },
}
</script>